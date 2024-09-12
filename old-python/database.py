from typing import List
from sqlalchemy import Date, create_engine, Column, Integer, String, ForeignKey, Text, Table, Engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship, sessionmaker, Session

# Define the Base class
Base = declarative_base()

# Define the association table for the many-to-many relationship
file_word_association = Table(
    'file_word', Base.metadata,
    Column('file_id', Integer, ForeignKey('files.id')),
    Column('word_id', Integer, ForeignKey('words.id'))
)

# Define the File class
class File(Base):
    __tablename__ = 'files'

    id = Column(Integer, primary_key=True)
    filename = Column(String, nullable=False)
    path = Column(String, nullable=False)
    filetype = Column(String, nullable=False)
    content = Column(Text, nullable=False)
    modified_date = Column(Date, nullable=False)

    # Relationship to the words (reverse index)
    words = relationship(
        'Word',
        secondary=file_word_association,
        back_populates='files'
    )

    def __repr__(self):
        return f"<File(filename='{self.filename}')>"

# Define the Word class
class Word(Base):
    __tablename__ = 'words'

    id = Column(Integer, primary_key=True)
    word = Column(String, nullable=False)

    # Relationship to the files
    files = relationship(
        'File',
        secondary=file_word_association,
        back_populates='words'
    )

    def __repr__(self):
        return f"<Word(word='{self.word}')>"

def add_file_with_words(session: Session, new_file: File, word_list: List[str]):
    # Add the new file to the session
    session.add(new_file)

    # Process each word in the word list
    for word_text in word_list:
        # Check if the word already exists in the database
        word = session.query(Word).filter_by(word=word_text).first()
        
        # If the word does not exist, create a new Word instance
        if not word:
            word = Word(word=word_text)
            session.add(word)
        
        # Associate the word with the new file
        new_file.words.append(word)

def file_indexed(session: Session, filename: str, path: str):
    return session.query(File).filter_by(filename=filename, path=path).first() is not None

# Database setup
def setup_database(database_url='sqlite:///search_engine.db'):
    engine = create_engine(database_url)
    Base.metadata.create_all(engine)
    return engine

# Create a new session
def create_session(engine: Engine):
    Session = sessionmaker(bind=engine)
    return Session()

# Example usage
if __name__ == '__main__':
    # Setup the database and create a session
    engine = setup_database()
    session = create_session(engine)

    # Create some example data
    file1 = File(filename='example1.txt', content='This is an example file.')
    file2 = File(filename='example2.txt', content='Another example file with some text.')

    word1 = Word(word='example')
    word2 = Word(word='file')
    word3 = Word(word='text')

    # Establish relationships
    file1.words.extend([word1, word2])
    file2.words.extend([word1, word2, word3])

    # Add to the session and commit
    session.add_all([file1, file2, word1, word2, word3])
    session.commit()

    # Query example
    for file in session.query(File).all():
        print(file)
        for word in file.words:
            print(f" - {word}")

    for word in session.query(Word).all():
        print(word)
        for file in word.files:
            print(f" - {file}")
