import urllib.request
import time

# Test the performance of the reverse proxy
proxy_endpoint = "http://localhost:4000"

tests = [10, 100, 1000, 5000, 10000]
f = open("performance.txt", "w")
f.write("Testing performance of kharon\nTime: {}\n\n".format(time.localtime(time.time())))
f.write("Performing the following requests:\n{}\n".format(", ".join([str(i) for i in tests])))

for n in tests:
  start = time.time()
  for i in range(n):
    urllib.request.urlopen(proxy_endpoint)
  elapsed = time.time() - start

  f.write("\n\nAttempting {} requests\n\t--\t{} s\n\t--\t{} req/s".format(n, round(elapsed, 6), round(n/elapsed, 6)))


f.close()