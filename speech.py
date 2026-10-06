"""Local voice bridge. Models are supplied by the user, never downloaded silently.
Requires sounddevice, soundfile, numpy, torch, silero-vad, kokoro.
whisper.cpp CLI is configured using WHISPER_CLI and WHISPER_MODEL.
"""
import os, sys, subprocess, pathlib

def capture(path):
    import sounddevice as sd
    import numpy as np
    import soundfile as sf
    import torch
    from silero_vad import load_silero_vad, VADIterator
    vad = VADIterator(load_silero_vad(), sampling_rate=16000)
    frames, started, quiet = [], False, 0
    # Blocks 32 ms; stop after 600 ms silence following speech, at most 30 s.
    with sd.InputStream(samplerate=16000, channels=1, dtype='float32', blocksize=512) as mic:
        for _ in range(938):
            block, overflow = mic.read(512)
            if overflow: raise RuntimeError('microphone overflow')
            event = vad(torch.from_numpy(block[:, 0].copy()))
            if event and 'start' in event: started = True
            if started: frames.append(block.copy())
            if event and 'end' in event: break
    if not frames: raise RuntimeError('no speech detected')
    sf.write(path, np.concatenate(frames), 16000)

def transcribe(audio, text, vocabulary):
    exe, model = os.environ.get('WHISPER_CLI'), os.environ.get('WHISPER_MODEL')
    if not exe or not model: raise RuntimeError('set WHISPER_CLI and WHISPER_MODEL')
    base = str(pathlib.Path(text).with_suffix(''))
    subprocess.run([exe, '-m', model, '-f', audio, '-otxt', '-of', base, '--prompt', vocabulary], check=True, timeout=110)

def speak(path):
    # Explicit local weights avoid Hugging Face downloads or telemetry during a turn.
    import sounddevice as sd
    from kokoro import KPipeline, KModel
    config, weights = os.environ.get('KOKORO_CONFIG'), os.environ.get('KOKORO_MODEL')
    voice = os.environ.get('KOKORO_VOICE_FILE')
    if not all((config, weights, voice)): raise RuntimeError('set local KOKORO_CONFIG, KOKORO_MODEL, KOKORO_VOICE_FILE')
    import torch
    model = KModel(config=config, model=weights)
    pipe = KPipeline(lang_code='a', model=model)
    for _, _, audio in pipe(pathlib.Path(path).read_text(), voice=torch.load(voice, weights_only=True)):
        sd.play(audio.numpy(), 24000); sd.wait()

if __name__ == '__main__':
    {'capture': capture, 'transcribe': transcribe, 'speak': speak}[sys.argv[1]](*sys.argv[2:])
