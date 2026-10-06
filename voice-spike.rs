mod voice {
// Local speech adapters. No shell is used; each stage receives explicit arguments.
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{path::Path, process::{Command, Stdio}, time::{Duration, Instant}};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Adapter { pub program: String, pub args: Vec<String> }
impl Adapter {
    pub fn run(&self, replacements: &[(&str, &str)], deadline: u64) -> Result<()> {
        let args: Vec<_> = self.args.iter().map(|a| replacements.iter().fold(a.clone(), |s,(k,v)|s.replace(k,v))).collect();
        let mut child = Command::new(&self.program).args(args).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().with_context(||format!("cannot start {}",self.program))?;
        let now=Instant::now();
        loop { if let Some(s)=child.try_wait()? { if !s.success(){bail!("{} exited {s}",self.program)} return Ok(()) }
            if now.elapsed()>Duration::from_secs(deadline){child.kill()?;child.wait()?;bail!("{} timed out",self.program)}
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceConfig {pub capture:Adapter,pub transcribe:Adapter,pub speak:Adapter,pub timeout_seconds:u64}
#[derive(Serialize)]
pub struct Timing {pub capture_ms:u128,pub stt_ms:u128,pub model_ms:u128,pub tts_ms:u128,pub transcript:String,pub reply:String}
pub fn turn(cfg:&VoiceConfig, input:Option<&Path>, vocabulary:&str, answer:impl FnOnce(&str)->Result<String>)->Result<Timing>{
    let dir=tempfile::tempdir()?;let wav=dir.path().join("speech.wav");let text=dir.path().join("transcript.txt");let reply_file=dir.path().join("reply.txt");
    let source=input.unwrap_or(&wav);let audio=source.to_str().context("non UTF8 audio path")?;let txt=text.to_str().unwrap();let output=reply_file.to_str().unwrap();
    let start=Instant::now();if input.is_none(){cfg.capture.run(&[("{audio}",audio)],cfg.timeout_seconds)?;}
    let capture_ms=start.elapsed().as_millis();let start=Instant::now();
    cfg.transcribe.run(&[("{audio}",audio),("{text}",txt),("{vocabulary}",vocabulary)],cfg.timeout_seconds)?;
    let transcript=std::fs::read_to_string(&text)?.trim().to_owned();if transcript.is_empty(){bail!("no speech recognized")}
    let stt_ms=start.elapsed().as_millis();let start=Instant::now();let reply=answer(&transcript)?;let model_ms=start.elapsed().as_millis();
    std::fs::write(&reply_file,&reply)?;let start=Instant::now();cfg.speak.run(&[("{reply}",output)],cfg.timeout_seconds)?;
    Ok(Timing{capture_ms,stt_ms,model_ms,tts_ms:start.elapsed().as_millis(),transcript,reply})
}
#[cfg(test)]mod tests{use super::*;#[test]fn missing_adapter_fails(){assert!(Adapter{program:"nonexistent-voxcode-command".into(),args:vec![]}.run(&[],1).is_err());}}

}
mod gateway {
use anyhow::{bail,Context,Result};
use serde::{Deserialize,Serialize};
use serde_json::json;
use std::time::Duration;
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Gateway {pub base_url:String,pub model:String,pub key_env:String,pub allow_cloud:bool,pub max_tokens:u32}
impl Default for Gateway{fn default()->Self{Self{base_url:"http://127.0.0.1:8080/v1".into(),model:"local".into(),key_env:"VOXCODE_API_KEY".into(),allow_cloud:false,max_tokens:1024}}}
impl Gateway{
 pub fn is_local(&self)->bool{reqwest::Url::parse(&self.base_url).ok().and_then(|u|u.host_str().map(str::to_string)).is_some_and(|h|matches!(h.as_str(),"127.0.0.1"|"localhost"|"[::1]"))}
 pub fn ask(&self,system:&str,user:&str)->Result<String>{
  if !self.is_local()&&!self.allow_cloud{bail!("cloud calls disabled; review project privacy and enable explicitly")}
  let client=reqwest::blocking::Client::builder().timeout(Duration::from_secs(120)).redirect(reqwest::redirect::Policy::none()).build()?;
  let mut req=client.post(format!("{}/chat/completions",self.base_url.trim_end_matches('/'))).json(&json!({"model":self.model,"max_tokens":self.max_tokens,"messages":[{"role":"system","content":system},{"role":"user","content":user}]}));
  if let Ok(key)=std::env::var(&self.key_env){req=req.bearer_auth(key);}
  let response:serde_json::Value=req.send()?.error_for_status()?.json()?;
  response["choices"][0]["message"]["content"].as_str().map(str::to_string).context("model returned no text")
 }
}
#[cfg(test)]mod tests{use super::*;#[test]fn cloud_gate(){let g=Gateway{base_url:"https://example.com/v1".into(),..Default::default()};assert!(!g.is_local());assert!(g.ask("","hi").is_err());}#[test]fn local_detect(){assert!(Gateway::default().is_local());}}

}
use anyhow::Result;
use clap::{Parser, Subcommand};

const OWNER: &str = env!("HARNESS_REPO_OWNER");
const REPO: &str = env!("HARNESS_REPO_NAME");
const BIN: &str = "harness";

#[derive(Parser)]
#[command(name = "harness", version, about = "Local-first voice agent harness for code projects")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Write a local example configuration (never writes keys)
    Init { path: std::path::PathBuf },
    /// One push-to-talk voice turn through local speech adapters
    Voice { config: std::path::PathBuf, #[arg(long)] audio: Option<std::path::PathBuf> },
    /// Detect OS and available processors without claiming GPU speed
    Doctor,
    /// Check for a newer release, optionally install it
    Update {
        /// Only check, do not install
        #[arg(long)]
        check: bool,
    },
}

fn target() -> &'static str {
    self_update::get_target()
}

fn updater() -> Result<Box<dyn self_update::update::ReleaseUpdate>> {
    Ok(self_update::backends::github::Update::configure()
        .repo_owner(OWNER)
        .repo_name(REPO)
        .bin_name(BIN)
        .target(target())
        .current_version(env!("CARGO_PKG_VERSION"))
        .show_download_progress(true)
        .no_confirm(true)
        .build()?)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Some(Cmd::Init{path})=>{if path.exists(){anyhow::bail!("config already exists")}
            std::fs::write(path,serde_json::to_string_pretty(&serde_json::json!({"gateway":gateway::Gateway::default(),"voice":{"timeout_seconds":120,"capture":{"program":"python3","args":["speech.py","capture","{audio}"]},"transcribe":{"program":"python3","args":["speech.py","transcribe","{audio}","{text}","{vocabulary}"]},"speak":{"program":"python3","args":["speech.py","speak","{reply}"]}}}))?)?;}
        Some(Cmd::Doctor)=>println!("OS: {} | arch: {} | CPUs: {} | GPU: not benchmarked; configure backend in your speech/model engine",std::env::consts::OS,std::env::consts::ARCH,std::thread::available_parallelism()?.get()),
        Some(Cmd::Voice{config,audio})=>{
            let v:serde_json::Value=serde_json::from_slice(&std::fs::read(config)?)?;
            let g:gateway::Gateway=serde_json::from_value(v["gateway"].clone())?;
            let cfg:voice::VoiceConfig=serde_json::from_value(v["voice"].clone())?;
            let t=voice::turn(&cfg,audio.as_deref(),"",|text|g.ask("Reply briefly. You are a coding assistant. Do not claim actions you did not run.",text))?;
            println!("{}",serde_json::to_string_pretty(&t)?);
        }
        Some(Cmd::Update { check }) => {
            let u = updater()?;
            let latest = u.get_latest_release()?;
            let cur = env!("CARGO_PKG_VERSION");
            if !self_update::version::bump_is_greater(cur, &latest.version)? {
                println!("up to date ({cur})");
                return Ok(());
            }
            println!("update available: {cur} -> {}", latest.version);
            if !check {
                let st = u.update()?;
                println!("updated to {}", st.version());
            }
        }
        None => println!("harness {} (voice UI and dashboard not built yet)", env!("CARGO_PKG_VERSION")),
    }
    Ok(())
}
