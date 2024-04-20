from flask import Flask, request

app = Flask(__name__)

@app.route('/post_endpoint', methods=['POST'])
def handle_post():
    data = request.data  # This is the data sent with the POST request
    return 'Data received', 200  # Response back to the client

if __name__ == '__main__':
    app.run(debug=True, port=8080)
