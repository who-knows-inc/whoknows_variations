from wsgiref.simple_server import make_server
from . import main  # Import main-funktionen fra din __init__.py

if __name__ == '__main__':
    app = main({})
    server = make_server('0.0.0.0', 8080, app)
    print("Server running on http://0.0.0.0:8080")
    server.serve_forever()
