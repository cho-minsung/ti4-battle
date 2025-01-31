FROM python:3.13-slim

WORKDIR /app

COPY . /app

RUN pip install --no-cache-dir -r requirements.txt

EXPOSE 8080

ENV NAME TI4-Battle-Simulator

CMD ["python", "main.py"]