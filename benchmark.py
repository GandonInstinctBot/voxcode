"""Benchmark actual configured voice runs. Does not download or call paid providers.
Usage: python benchmark.py /path/to/harness config.json fixtures.json
fixtures: [{"audio":"...wav","expected":"spoken transcript"}]
Select CPU/Metal/CUDA/Vulkan in the speech engine itself, run once per backend.
"""
import json,subprocess,sys,time
try: import resource
except ImportError: resource=None

def distance(a,b):
 row=list(range(len(b)+1))
 for i,x in enumerate(a,1):
  nxt=[i]
  for j,y in enumerate(b,1):nxt.append(min(nxt[-1]+1,row[j]+1,row[j-1]+(x!=y)))
  row=nxt
 return row[-1]

results=[]
for f in json.load(open(sys.argv[3])):
 start=time.monotonic();p=subprocess.run([sys.argv[1],'voice',sys.argv[2],'--audio',f['audio']],text=True,capture_output=True)
 if p.returncode:results.append({'audio':f['audio'],'error':p.stderr});continue
 j=json.loads(p.stdout);expected=f['expected'].lower().split();got=j['transcript'].lower().split();j['word_error_rate']=distance(expected,got)/max(1,len(expected));j['wall_seconds']=time.monotonic()-start;j['audio']=f['audio'];results.append(j)
print(json.dumps({'runs':results,'max_child_rss_native_units':resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss if resource else None,'note':'RSS units differ across OS. Windows resource module unavailable; use Task Manager or platform profiler. Backend is configured externally. No performance claim without real fixtures.'},indent=2))
