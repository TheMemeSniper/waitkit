#!/usr/bin/env python3

# Example WaitKit C2 + RS server

from http.server import BaseHTTPRequestHandler, HTTPServer

command = b"echo WaitKit"

class MyServer(BaseHTTPRequestHandler):

    def do_GET(self):
        print("GET request received")
        print("Path:", self.path)
        print("Headers:", self.headers)

        content_length = int(self.headers.get("Content-Length", 0))
        if content_length > 0:
            body = self.rfile.read(content_length)
            print("Body:", body)

        self.send_response(200)
        self.send_header("Content-type", "text/plain")
        self.end_headers()
        self.wfile.write(command)

    def do_POST(self):
        print("POST request received")
        print("Path:", self.path)
        print("Headers:", self.headers)

        content_length = int(self.headers.get("Content-Length", 0))
        if content_length > 0:
            body = self.rfile.read(content_length)
            print("Body:", body)

        self.send_response(200)
        self.send_header("Content-type", "text/plain")
        self.end_headers()
        self.wfile.write(b"WaitKit Response Server")

def main():
    PORT = 8000
    server = HTTPServer(("", PORT), MyServer)
    print("Server listening on port", PORT)
    server.serve_forever()

if __name__ == "__main__":
    main()

