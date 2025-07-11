#!/usr/bin/env python3
 
from hdrh.histogram import HdrHistogram
import matplotlib.ticker as ticker
import matplotlib.pyplot as plt
import pandas as pd

MSECS_PER_SEC = 1000

histogram = HdrHistogram(1, MSECS_PER_SEC, 4)

df = pd.read_csv('ping-google.com.csv')
for latency in df["Latency_secs"]:
    histogram.record_value(latency * MSECS_PER_SEC)

data = []
for percentile in [25.0, 50.0, 90.0, 99.0, 99.9, 99.99]:
    value = histogram.get_value_at_percentile(percentile)
    data.append([percentile / 100, value])

hist_df = pd.DataFrame(data, columns=['Percentile', 'Value'])
fig, ax = plt.subplots()
ax.plot(hist_df["Percentile"], hist_df["Value"])
ax.grid()
ax.set_title('Latency by Percentile Distribution')
ax.set_xlabel('Percentile (%)')
ax.set_ylabel('Latency (milliseconds)')
ax.set_xscale('logit')
plt.xticks([0.25, 0.5, 0.9, 0.99, 0.999, 0.9999])
majors = ["25%", "50%", "90%", "99%", "99.9%", "99.99%"]
ax.xaxis.set_major_formatter(ticker.FixedFormatter(majors))
ax.xaxis.set_minor_formatter(ticker.NullFormatter())
fig.savefig('histogram-hdr.png', dpi=300)
