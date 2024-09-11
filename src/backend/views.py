from pyramid.view import view_config
from pyramid.response import Response
from pyramid.renderers import render_to_response
from .models import connect_db, query_db, hash_password, verify_password
import json

# Home Page
@view_config(route_name='home', renderer='json')
def home_view(request):
    return {'message': 'Welcome to the Home Page'}

# Search page
@view_config(route_name='search', renderer='templates/search.html.jinja2')
def search_view(request):
    q = request.params.get('q', None)
    language = request.params.get('language', "en")
    search_results = []
    if q:
        search_results = query_db("SELECT * FROM pages WHERE language = ? AND content LIKE ?", (language, f"%{q}%"))
    return {'search_results': search_results, 'query': q}

# Login page
@view_config(route_name='login', renderer='templates/login')
def login_view(request):
    if 'user_id' in request.session:
        return Response('Already logged in')

    if request.method == 'POST':
        username = request.params.get('username')
        password = request.params.get('password')

        # Ensure username and password are provided
        if not username or not password:
            return {'error': 'Username and password are required'}, 400

        # Query the user from the database
        user = query_db("SELECT * FROM users WHERE username = ?", (username,), one=True)

        # Validate the user and password
        if user and verify_password(user['password'], password):
            request.session['user_id'] = user['id']
            return Response('Logged in successfully')
        else:
            return {'error': 'Invalid login'}, 401  # Unauthorized access
    
    return {}


# Register page
@view_config(route_name='register', renderer='templates/register.html.jinja2')
def register_view(request):
    if 'user_id' in request.session:
        return Response('Already registered')

    if request.method == 'POST':
        username = request.params.get('username')
        email = request.params.get('email')
        password = request.params.get('password')
        password2 = request.params.get('password2')

        if not username:
            return {'error': 'Username is required'}
        if not email or '@' not in email:
            return {'error': 'A valid email is required'}
        if not password:
            return {'error': 'Password is required'}
        if password != password2:
            return {'error': 'Passwords do not match'}
        if query_db("SELECT * FROM users WHERE username = ?", (username,), one=True):
            return {'error': 'Username is already taken'}
        
        # Insert user into database
        conn = connect_db()
        conn.execute("INSERT INTO users (username, email, password) VALUES (?, ?, ?)", 
                     (username, email, hash_password(password)))
        conn.commit()
        conn.close()
        return Response('Successfully registered!')

    return {}

# Logout
@view_config(route_name='logout')
def logout_view(request):
    request.session.invalidate()
    return Response('Logged out successfully')

# API Login
@view_config(route_name='api_login', request_method='POST', renderer='json')
def api_login_view(request):
    username = request.params.get('username')
    password = request.params.get('password')
    user = query_db("SELECT * FROM users WHERE username = ?", (username,), one=True)
    
    if user and verify_password(user['password'], password):
        request.session['user_id'] = user['id']
        return {'message': 'Logged in successfully'}
    
    return {'error': 'Invalid login'}, 401

# API Register
@view_config(route_name='api_register', request_method='POST', renderer='json')
def api_register_view(request):
    username = request.params.get('username')
    email = request.params.get('email')
    password = request.params.get('password')
    password2 = request.params.get('password2')

    if not username:
        return {'error': 'Username is required'}, 400
    if not email or '@' not in email:
        return {'error': 'A valid email is required'}, 400
    if not password:
        return {'error': 'Password is required'}, 400
    if password != password2:
        return {'error': 'Passwords do not match'}, 400
    if query_db("SELECT * FROM users WHERE username = ?", (username,), one=True):
        return {'error': 'Username is already taken'}, 400
    
    # Insert user into database
    conn = connect_db()
    conn.execute("INSERT INTO users (username, email, password) VALUES (?, ?, ?)", 
                 (username, email, hash_password(password)))
    conn.commit()
    conn.close()

    return {'message': 'Successfully registered'}, 201  # Created

# API Logout
@view_config(route_name='logout')
def logout_view(request):
    if 'user_id' in request.session:
        request.session.invalidate()
        return Response('Logged out successfully')
    else:
        return Response('No active session to log out from', status=400)


@view_config(route_name='weather', renderer='json')
def weather_view(request):
    # Add logic to serve weather data
    return {'message': 'Weather data'}

@view_config(route_name='api_search', renderer='json')
def api_search_view(request):
    # Add logic to handle search
    query = request.params.get('q', None)
    return {'message': f'Searching for: {query}'}

@view_config(route_name='api_weather', renderer='json')
def api_weather_view(request):
    # Add logic to serve weather data as API
    return {'weather': 'sunny'}

