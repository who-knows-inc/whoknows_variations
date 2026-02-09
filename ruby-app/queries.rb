require 'sqlite3'

Page = Struct.new(:id, :title, :language, :content)

begin
  db = SQLite3::Database.new "whoknows.db"

  def insert_user_query(db)
    query = "INSERT INTO users (username, email, password) VALUES ('johndoe', 'john@example.com', '5f4dcc3b5aa765d61d8327deb882cf99')"
    db.execute(query)
    db.last_insert_row_id
  end

  def get_user_id_query(db)
    query = "SELECT id FROM users WHERE username = 'johndoe'"
    row = db.get_first_row(query)
    row ? row[0] : nil
  end

  def get_user_by_id_query(db)
    query = "SELECT * FROM users WHERE id = '1'"
    row = db.get_first_row(query)
    if row
      id, username, email, password = row
      [id, username, email, password]
    else
      [nil, nil, nil, nil]
    end
  end

  def get_user_by_username_query(db)
    query = "SELECT * FROM users WHERE username = 'johndoe'"
    row = db.get_first_row(query)
    if row
      id, username, email, password = row
      [id, username, email, password]
    else
      [nil, nil, nil, nil]
    end
  end

  def search_pages_query(db)
    query = "SELECT * FROM pages WHERE language = 'en' AND content LIKE '%golang%'"
    pages = []
    db.execute(query) do |row|
      id, title, language, content = row
      pages << Page.new(id, title, language, content)
    end
    pages
  end

  last_id = insert_user_query(db)
  if last_id
    puts "InsertUserQuery: Inserted user with id #{last_id}"
  else
    puts "InsertUserQuery error"
  end

  user_id = get_user_id_query(db)
  if user_id
    puts "GetUserIDQuery: User 'johndoe' has id #{user_id}"
  else
    puts "GetUserIDQuery error"
  end

  id, username, email, password = get_user_by_id_query(db)
  if id
    puts "GetUserByIDQuery: id=#{id} username=#{username} email=#{email} password=#{password}"
  else
    puts "GetUserByIDQuery error"
  end

  id, username, email, password = get_user_by_username_query(db)
  if id
    puts "GetUserByUsernameQuery: id=#{id} username=#{username} email=#{email} password=#{password}"
  else
    puts "GetUserByUsernameQuery error"
  end

  pages = search_pages_query(db)
  if pages
    pages.each do |page|
      puts "SearchPagesQuery: id=#{page.id} title=#{page.title} language=#{page.language} content=#{page.content}"
    end
  else
    puts "SearchPagesQuery error"
  end

rescue SQLite3::Exception => e
  puts "Database error: #{e}"
ensure
  db.close if db
end