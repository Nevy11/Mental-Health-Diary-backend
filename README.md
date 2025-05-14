`PROJECT DESCRIPTION` 

A server for Mental health diary web application. Build using rust's actix server to enhance perfomance and 
saving memory. Used for interaction with the front end.

`SETTING UP`

The project is a rust's project, to use it, first install Cargo and rust.
clone the repository then run cargo run to build and run the project on the server.
You could link the api's with you're front end.

`Example:` 

to transcribe audio to text:
"http://0.0.0.0:8080/transcribe"
add the api endpoint to you're project
the api returns a json file of: 
{success: true, message: transcribed_message}
there you have it, you're audio is transcribed :)

`HOW TO USE IT`

Make sure you have rust and cargo installed in your machine before you start
1. Install postgresql
   
          https://www.postgresql.org/download/
2. Enter this in postgresql 

         ALTER DATABASE chat_application_database REFRESH COLLATION VERSION;
3. Create the database and fill in the `yourusername` with your username and `yourpassword` with your actual password:

         CREATE USER yourusername WITH PASSWORD 'yourpassword';
         CREATE DATABASE chat_application_database OWNER yourusername;
4. create a .env file and in it fill in the following

         DATABASE_URL=postgres://yourusername:yourpassword@127.0.0.1/chat_application_database
         SECRET_KEY=yourpassword
5. clone the repository

       git clone git@github.com:Nevy11/Mental-Health-Diary-backend.git
6. Enter the folder where you have cloned the repository

         cd Mental-Health-Diary-backend/
7. build the project using cargo
   
       Cargo build
8. Run the project
  
        cargo run
    
