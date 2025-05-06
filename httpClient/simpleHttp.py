from http.server import BaseHTTPRequestHandler, HTTPServer
import time

hostName = "localhost"
serverPort = 8080
counter = 0

class MyServer(BaseHTTPRequestHandler):
    def do_GET(self):
        
        global counter
        if self.path == "/favicon.ico":
            self.send_response(404)
            self.end_headers()
            return
        counter += 1
        self.send_response(200)
        self.send_header("Content-type", "text/html")
        self.end_headers()
        self.wfile.write(bytes(str(counter), "utf-8"))
        print("Request received, path: ", self.path, " count: ", counter)
    
    def log_message(self, format, *args):
        return # disable logging

myServer = HTTPServer((hostName, serverPort), MyServer)
print(time.asctime(), "Server Starts - %s:%s" % (hostName, serverPort))

try:
    myServer.serve_forever()
except KeyboardInterrupt:
    pass

myServer.server_close()
print(time.asctime(), "Server Stops - %s:%s" % (hostName, serverPort))