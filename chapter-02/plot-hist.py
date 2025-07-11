#!/usr/bin/env python3

import matplotlib.pyplot as plt
import pandas as pd

df = pd.read_csv('ping-google.com.csv')
plt.hist(df["Latency_secs"] * 1000, bins='auto')
plt.ylabel("Frequency (N = %d)" % len(df))
plt.xlabel('Latency (in milliseconds)')
plt.savefig('histogram.png', dpi=300)
