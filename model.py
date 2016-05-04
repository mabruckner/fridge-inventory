from sqlalchemy import create_engine,Column,Integer,String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker


Base = declarative_base()


class Item(Base):
    __tablename__ = 'items'
    id = Column(Integer, primary_key=True)
    name = Column(String(256))

engine = create_engine('sqlite:///fridge.db')

Base.metadata.create_all(engine)

DBSession = sessionmaker(bind=engine)
