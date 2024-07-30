FROM python:3.8-slim
EXPOSE 8080

WORKDIR /usr/src/app

COPY . .

# Keeps Python from generating .pyc files in the container
ENV PYTHONDONTWRITEBYTECODE=1

# Install dependencies needed for compiling some Python packages
RUN apt-get update && apt-get install -y \
    build-essential \
    libpq-dev \
    && apt-get clean

# Install Python dependencies
RUN pip install --no-cache-dir -r requirements.txt


CMD ["python", "app.py"]