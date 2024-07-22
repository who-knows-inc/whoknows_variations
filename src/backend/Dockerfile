FROM python:3.8-slim
EXPOSE 8080

WORKDIR /usr/src/app

COPY . .

# Keeps Python from generating .pyc files in the container
ENV PYTHONDONTWRITEBYTECODE=1

# Turns off buffering for easier container logging
ENV PYTHONUNBUFFERED=1

# Update package lists and install psycopg2 dependencies
RUN apt-get update -y && apt-get install -y \
    libpq-dev \
    gcc \
    postgresql-client \
 && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Creates a non-root user with an explicit UID and adds permission to access the /usr/src/app folder
RUN adduser --disabled-password --gecos "" --uid 5678 appuser && \
    chown -R appuser /usr/src/app && chown -R appuser /tmp

# Switch to the non-root user
USER appuser

CMD ["python", "app.py"]