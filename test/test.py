# from  urllib import request
import requests
import time
import json

# Test the performance of the reverse proxy
ENDPOINT = "http://localhost:4000"

# tests = [10, 100, 1000, 5000, 10000]
f = open("performance.txt", "w")
f.write("Testing performance of kharon\nTime: {}\n\n".format(time.localtime(time.time())))
# f.write("Performing the following requests:\n{}\n".format(", ".join([str(i) for i in tests])))

# for n in tests:
  # start = time.time()
  # for i in range(n):
    # request.urlopen(proxy_endpoint)
    # request.
  # elapsed = time.time() - start
# 
  # f.write("\n\nAttempting {} requests\n\t--\t{} s\n\t--\t{} req/s".format(n, round(elapsed, 6), round(n/elapsed, 6)))

tests = json.load(open("tests.json", "r"))

for key in tests.keys():
  print("Beginning {}\n{}".format(key, tests[key]['description']))

  for (i, req) in enumerate(tests[key]['requests']):
  
    # Build requests from fields in request element
    if req['method'] == "GET":
      url = ENDPOINT + (req['route'])
      r = requests.get(url=url, params=req['params'])
      print(r.headers)
    else:
      pass


f.close()