from sqlalchemy import Column, Integer, String, Text, DateTime, CheckConstraint
from sqlalchemy.orm import declarative_base
from sqlalchemy.sql import func

Base = declarative_base()


class User(Base):
    __tablename__ = 'users'

    id = Column(Integer, primary_key=True)
    username = Column(Text, nullable=False, unique=True)
    email = Column(Text, nullable=False, unique=True)
    password = Column(Text, nullable=False)


class Page(Base):
    __tablename__ = 'pages'

    title = Column(Text, primary_key=True)
    url = Column(Text, nullable=False, unique=True)
    language = Column(Text, CheckConstraint("language IN ('en', 'da')"), nullable=False, default='en')
    last_updated = Column(DateTime, default=func.now(), onupdate=func.now())
    content = Column(Text, nullable=False)
