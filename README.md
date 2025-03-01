`PROJECT DESCRIPTION` 
A server for Mental health diary web application. Build using rust's actix server to enhance perfomance and 
saving memory. Used for interaction with the front end.

`Setting up`
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

