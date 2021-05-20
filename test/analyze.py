"""
This Python module serves to analyze the performance metrics of kharon. 

"""
#%%
import matplotlib.pyplot as plt
import sys
import os
import numpy as np
from scipy.stats import norm
import statistics

# retrieve data from
option = -1 
if sys.argv[1] == "new":
  option = 0
else:
  option = 1

TEST_SIZE = 1000

sample_averages = []

if os.path.exists("means.txt"):
  os.remove("means.txt")

for i in range(TEST_SIZE + 1):

  # if (i != 0 and i % 5 == 0) or i == TEST_SIZE - 2:
  #   mf = open("means.txt", "a")
  #   mf.write("{}\n".format(sample_averages))
  #   sample_averages = []
  #   mf.close()

  # if i == TEST_SIZE - 1:
  #   break

  os.system("ab -n 10000 -c 500 http://localhost:4000/ > performance_threadpool.txt")
  pf = open("performance_threadpool.txt", "r")
  # The mean is on the 21st line
  for i in range(20):
    pf.readline()

  mean_list = pf.readline().split()

  if len(mean_list) > 4:
    mean = float(mean_list[3])
    sample_averages.append(mean)
  pf.close()

x_axis = np.arange(4000, 10000, 0.5)
print(sample_averages)
mu = statistics.mean(data=sample_averages)
sd = statistics.stdev(data=sample_averages)

plt.plot(x_axis, norm.pdf(x_axis, mu, sd))

# with open("means.txt") as openfileobject:

#   for line in openfileobject:
#     # plt.plot(line.strip('][').split(', '))
    


# plt.plot([1,2,3,4])
# plt.ylabel('some numbers')
plt.show()



# %%
