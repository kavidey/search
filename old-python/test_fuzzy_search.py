import sqlalchemy as sa
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

# Define the database and table structure
Base = declarative_base()

class Document(Base):
    __tablename__ = 'documents'
    id = Column(Integer, primary_key=True, autoincrement=True)
    content = Column(String)

# Create an SQLite engine
engine = create_engine('sqlite:///fts_spellfix.db', echo=True)

# Path to the spellfix1 extension shared library
spellfix1_extension_path = 'spellfix.dylib'  # Update this path as needed

# Connect to the database using a raw connection to load the extension and create virtual tables
conn = engine.raw_connection()
try:
    conn.enable_load_extension(True)
    cursor = conn.cursor()
    cursor.execute(f"SELECT load_extension('{spellfix1_extension_path}');")
    cursor.execute('CREATE VIRTUAL TABLE IF NOT EXISTS fts_documents USING fts5(content);')
    cursor.execute('CREATE VIRTUAL TABLE IF NOT EXISTS spellfix1 USING spellfix1;')
    conn.commit()
finally:
    conn.enable_load_extension(False)
    conn.close()

# Create the table
Base.metadata.create_all(engine)

# Create a session
Session = sessionmaker(bind=engine)
session = Session()

# Insert sample data
documents = [
    Document(content="This is a sample document"),
    Document(content="Another example document"),
    Document(content="SQLite with Full-Text Search"),
    Document(content="Using Spellfix for fuzzy search"),
]

session.add_all(documents)
session.commit()

# Populate the FTS table
conn = engine.raw_connection()
try:
    cursor = conn.cursor()
    cursor.execute('INSERT INTO fts_documents (rowid, content) SELECT id, content FROM documents;')
    conn.commit()
finally:
    conn.close()

# Populate the Spellfix table with words from documents
conn = engine.raw_connection()
try:
    cursor = conn.cursor()
    for doc in documents:
        words = doc.content.split()
        for word in words:
            cursor.execute('INSERT INTO spellfix1(word) VALUES (?)', (word,))
    conn.commit()
finally:
    conn.close()

# Function to perform fuzzy search using FTS and Spellfix
def fuzzy_search(query):
    print(query)
    conn = engine.raw_connection()
    try:
        cursor = conn.cursor()
        corrected_word = cursor.execute(
            'SELECT word FROM spellfix1 WHERE word MATCH ? LIMIT 1', (query,)
        ).fetchone()

        print(corrected_word)
        
        if corrected_word:
            corrected_word = corrected_word[0]
        else:
            corrected_word = query

        results = cursor.execute(
            'SELECT content FROM fts_documents WHERE content MATCH ?', (corrected_word,)
        ).fetchall()
        
        return [result[0] for result in results]
    finally:
        conn.close()

# Example fuzzy search
query = "docment"
results = fuzzy_search(query)
print(f"Search results for '{query}':")
for result in results:
    print(result)
