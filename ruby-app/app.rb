require 'sinatra'
require 'sqlite3'
require 'json'

get '/' do
  'Hello world!!!!!!'
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
