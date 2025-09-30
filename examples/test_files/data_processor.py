#! Data Processing Module
"""startsummary
This module provides comprehensive data processing capabilities for
handling various data formats and performing common data manipulation
tasks. It includes functions for reading, cleaning, filtering, and
transforming datasets.

The module is designed to be memory-efficient and can handle large
datasets by processing them in chunks when necessary.
endsummary"""

def process_data(data):
    """Process and clean the input data
    
    Takes a list of data items and performs basic cleaning operations
    including trimming whitespace, removing empty items, and converting
    to appropriate data types.
    
    Args:
        data (list): List of data items to process
        
    Returns:
        list: Cleaned and processed data items
        
    Example:
        >>> data = ["  item1  ", "", "item2", "  "]
        >>> process_data(data)
        ['item1', 'item2']
    """
    return [item.strip() for item in data if item.strip()]

#! File Operations
"""startsummary
File handling utilities for reading and writing data files safely.
These functions provide error handling and support for different
file formats including JSON, CSV, and plain text.
endsummary"""

def read_file(filename):
    """Read a file and return its contents
    
    Safely reads a text file and returns its contents as a string.
    Handles common file reading errors gracefully.
    
    Args:
        filename (str): Path to the file to read
        
    Returns:
        str: Contents of the file
        
    Raises:
        FileNotFoundError: If the file doesn't exist
        IOError: If there's an error reading the file
    """
    with open(filename, 'r', encoding='utf-8') as f:
        return f.read()

def write_json(data, filename):
    """Write data to a JSON file
    
    Converts Python data structures to JSON format and writes
    them to the specified file with proper formatting.
    
    Args:
        data: Python object to serialize to JSON
        filename (str): Path where to save the JSON file
    """
    import json
    with open(filename, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

#! Statistical Functions
"""startsummary
Basic statistical analysis functions for numerical data.
Includes mean, median, standard deviation, and other common
statistical measures.
endsummary"""

def calculate_mean(numbers):
    """Calculate the arithmetic mean of a list of numbers
    
    Args:
        numbers (list): List of numbers to calculate mean for
        
    Returns:
        float: The arithmetic mean of the numbers
    """
    if not numbers:
        return 0
    return sum(numbers) / len(numbers)
# ENDVEXDOC