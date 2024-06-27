import re
from typing import List
import spacy

nlp = spacy.load("en_core_web_sm")

def clean_up_word_list(words: List[str], max_len=10):
    words = [word for word in words if len(word) <= max_len]

    text = " ".join(words).lower()
    doc = nlp(text)

    filtered_words = [token.lemma_ for token in doc if not token.is_stop]

    return filtered_words

def shorten_path(path: str, maxlen=75):
    import os
    
    # Split the path into components
    parts = path.split(os.sep)
    
    # If the path has less than 3 parts, return as is
    if len(parts) <= 2:
        return path
    
    # Combine first, '...' and the last part
    shortened_path = os.sep.join([parts[0], parts[1], '...', parts[-2], parts[-1]])

    constant_len_path = shortened_path.ljust(maxlen)[:maxlen]
    
    return constant_len_path

def flatten(matrix: List[List]):
    return [item for row in matrix for item in row]

camelCase = re.compile("(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])")
delimiters = [" ", ".", "_", "-"]
# delimitersSplit = "|".join("(?<={})".format(re.escape(delim)) for delim in delimiters)
delimitersSplit = "|".join("{}".format(re.escape(delim)) for delim in delimiters)
delimitersSplit = re.compile(delimitersSplit)

def split_file_name(string: str):
    cc = re.split(camelCase, string)
    splt = [re.split(delimitersSplit, part) for part in cc]

    return flatten(splt)