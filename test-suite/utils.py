#!/usr/bin/env python3
"""
Utility functions with additional dependencies for testing PyWand.
"""

import os
import re
import json
import logging
from typing import Dict, List, Any, Optional

# External dependencies
import yaml
import requests
import boto3
from rich import print as rprint
from pydantic import BaseModel, Field

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class Config(BaseModel):
    """Configuration model using Pydantic."""
    app_name: str
    version: str
    debug: bool = False
    api_key: Optional[str] = None
    endpoints: Dict[str, str] = Field(default_factory=dict)
    
def load_config(config_path: str) -> Config:
    """Load configuration from YAML file."""
    try:
        if not os.path.exists(config_path):
            logger.error(f"Config file not found: {config_path}")
            return Config(app_name="default", version="0.1.0")
            
        with open(config_path, 'r') as f:
            config_data = yaml.safe_load(f)
            
        return Config(**config_data)
    except Exception as e:
        logger.error(f"Error loading config: {str(e)}")
        return Config(app_name="default", version="0.1.0")
        
def fetch_data_from_s3(bucket: str, key: str) -> Dict[str, Any]:
    """Fetch data from AWS S3."""
    try:
        s3_client = boto3.client('s3')
        response = s3_client.get_object(Bucket=bucket, Key=key)
        data = json.loads(response['Body'].read().decode('utf-8'))
        return data
    except Exception as e:
        logger.error(f"Error fetching data from S3: {str(e)}")
        return {}
        
def format_output(data: Dict[str, Any]) -> None:
    """Format and print output using rich."""
    rprint("[bold green]Data Output:[/bold green]")
    rprint(data)
    
def main():
    """Main function to demonstrate utilities."""
    logger.info("Starting utility demonstration")
    
    # Load config
    config = load_config("config.yaml")
    logger.info(f"Loaded configuration for: {config.app_name} v{config.version}")
    
    # Print formatted output
    sample_data = {
        "name": "PyWand",
        "type": "Utility",
        "dependencies": ["pydantic", "boto3", "rich", "yaml"]
    }
    format_output(sample_data)
    
    logger.info("Utility demonstration completed")
    
if __name__ == "__main__":
    main() 