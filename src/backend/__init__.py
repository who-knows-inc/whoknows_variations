from pyramid.config import Configurator
from pyramid.session import SignedCookieSessionFactory
import pyramid_openapi3  # Make sure to import this

def main(global_config, **settings):
    config = Configurator(settings=settings)

    # Set up session factory
    session_factory = SignedCookieSessionFactory('itsaseekreet')  # Secret key for signing cookies
    config.set_session_factory(session_factory)
    
    config.include('pyramid_jinja2')
    config.include('pyramid_openapi3')  # Include this line

    # Load your OpenAPI specification file
    config.pyramid_openapi3_spec('backend/openapi.json', route='/openapi.json')


    # Optional: Serve Swagger UI or Redoc at /docs
    config.pyramid_openapi3_add_explorer(route='/docs')

    # Serve static files
    config.add_static_view(name='static', path='myapp:static')

    # Define routes
    config.add_route('home', '/')
    config.add_route('search', '/search')
    config.add_route('login', '/login')
    config.add_route('logout', '/logout')
    config.add_route('register', '/register')
    config.add_route('api_login', '/api/login')
    config.add_route('api_register', '/api/register')
    config.add_route('api_logout', '/api/logout')
    config.add_route('weather', '/weather')
    config.add_route('api_search', '/api/search')
    config.add_route('api_weather', '/api/weather')
  

    # Scan for view configurations
    config.scan()

    return config.make_wsgi_app()
