#!/usr/bin/env python3
import secrets
import string
import argparse

def generate_api_key(length=32):
    """Generate a secure API key of specified length."""
    alphabet = string.ascii_letters + string.digits
    api_key = ''.join(secrets.choice(alphabet) for _ in range(length))
    return api_key

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a secure API key")
    parser.add_argument("-l", "--length", type=int, default=32,
                        help="Length of the API key (default: 32)")
    parser.add_argument("-p", "--prefix", type=str, default="",
                        help="Optional prefix for the API key")
    
    args = parser.parse_args()
    
    api_key = args.prefix + generate_api_key(args.length)
    print(api_key) 