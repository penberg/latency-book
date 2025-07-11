#!/usr/bin/env python3

import matplotlib.ticker as tck
import matplotlib.pyplot as plt
import statsmodels.api as sm
import pandas as pd
import numpy as np

df = pd.read_csv('ping-google.com.csv')
values = df["Latency_secs"] * 1000
ecdf = sm.distributions.ECDF(values)
x = np.linspace(min(values), max(values))
y = ecdf(x)
fig, ax = plt.subplots()
plt.plot(x, y)
ax.xaxis.set_minor_locator(tck.AutoMinorLocator())
plt.grid()
plt.xlim((min(x), max(x)))
plt.xlabel('Latency (in milliseconds)')
plt.ylabel('Cumulative density')
plt.yticks(np.arange(0, 1.1, 0.1))
plt.savefig('ecdf.png', dpi=300)
