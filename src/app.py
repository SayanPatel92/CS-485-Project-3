from flask import Flask, request

app = Flask(__name__)

@app.route('/', methods=['POST'])
def upload_file():
    # Get binary data from request body
    data = request.data
    
    # You can now do something with the data, like save it to a file
    with open('uploaded_file', 'wb') as f:
        f.write(data)

    # Return a response
    return 'File uploaded successfully', 200

if __name__ == '__main__':
    app.run(debug=True, host='127.0.0.1', port=8000)
