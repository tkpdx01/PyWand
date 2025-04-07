#!/usr/bin/env python3
"""
Sample Python script with various dependencies for testing PyWand.
"""

import os
import sys
import json
from pathlib import Path
import datetime

# External dependencies
import requests
import numpy as np
import pandas as pd
from flask import Flask, jsonify
import matplotlib.pyplot as plt
from sqlalchemy import create_engine

def main():
    """Main function that demonstrates the use of various libraries."""
    print("Sample Python script with dependencies")
    
    # Using requests
    response = requests.get("https://httpbin.org/json")
    data = response.json()
    print(f"HTTP Request Status: {response.status_code}")
    
    # Using numpy and pandas
    df = pd.DataFrame({
        'A': np.random.rand(5),
        'B': np.random.rand(5)
    })
    print("\nSample DataFrame:")
    print(df)
    
    # Using matplotlib (commented out for headless environments)
    # plt.figure(figsize=(10, 6))
    # plt.plot(df['A'], label='Series A')
    # plt.plot(df['B'], label='Series B')
    # plt.legend()
    # plt.title('Sample Plot')
    # plt.savefig('sample_plot.png')
    
    # Using Flask (just import, not running)
    app = Flask(__name__)
    print("\nFlask app created")
    
    # Using SQLAlchemy (just creating engine, not connecting)
    engine = create_engine('sqlite:///:memory:')
    print("SQLAlchemy engine created")
    
    print("\nAll libraries imported successfully!")

if __name__ == "__main__":
    main() 