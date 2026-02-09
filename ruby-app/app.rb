require 'sinatra'

get '/' do
  'Hello world!'
end


get '/about' do
    erb :about
end