`PROJECT DESCRIPTION` 
A server for Mental health diary web application. Build using rust's actix server to enhance perfomance and 
saving memory. Used for interaction with the front end.

`Setting up`
The project is a rust's project, to use it, first install Cargo and rust.
clone the repository then run cargo run to build and run the project on the server.
You could link the api's with you're front end.

`Example:` \n
to transcribe audio to text:
"http://0.0.0.0:8080/transcribe"
add the api endpoint to you're project
the api returns a json file of: 
{success: true, message: transcribed_message}
there you have it, you're audio is transcribed :)

`How to use it`
Make sure you have rust and cargo installed in your machine before you start
1. clone the repository

       git clone git@github.com:Nevy11/Mental-Health-Diary-backend.git
2. Enter the folder where you have cloned the repository

         cd Mental-Health-Diary-backend/
3. build the project using cargo
   
       Cargo build
4. Run the project
  
        cargo run
    
