require 'sinatra'
require 'sqlite3'
require 'json'

#Shows the search page
get '/' do

  # retrieves the query parameter from the url
  query = params[:q]

  # retrieves the languge parameter (defaults to english)
  language = params[:language] || 'en'

  # if no query, return no 
  unless query
    search_results = []
  else
    # retrieve from database
    search_results = search_pages_query(db, language, query)  
  end 

  # always renders the search template, with or without a populated result array
  erb :search, locals: { search_results: search_results, query: query }
end

# VIEWS
get '/about' do
    erb :about
end

get '/login' do
    erb :login
end

get '/register' do
    erb :register
end

# DATABASE
def get_db
    SQLite3::Database.new 'whoknows.db'
end

# ENDPOINTS   
get '/api/users' do
    content_type :json
    db = get_db
    users = []
  
  db.execute("SELECT id, username, email FROM users") do |row|
    users << { id: row[0], username: row[1], email: row[2] }
  end
  
  db.close
  users.to_json
end

post '/api/login' do
    content_type :json

    {
      message: "Login endpoint hit"
    }.to_json
end

post '/api/register' do
    content_type :json

    {
      message: "Register endpoint hit"
    }.to_json
end