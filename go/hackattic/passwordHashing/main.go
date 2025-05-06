package main

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"golang.org/x/crypto/pbkdf2"
)


func main() {
  
  resp,err := http.Get("https://hackattic.com/challenges/password_hashing/problem?access_token=9c79a59dcd7d2cba")
  if( err != nil) {
    return
  }
  defer resp.Body.Close()
  body,_ := io.ReadAll(resp.Body)
  var dat map[string]interface{}
  json.Unmarshal(body,&dat)
  
  var password  = dat["password"]
  salt,_ := base64.StdEncoding.DecodeString(dat["salt"].(string)) 
  var x = dat["pbkdf2"].(map[string]interface{})
  // var hash = x["hash"]
  rounds := int(x["rounds"].(float64))
 
  // var scrypt = dat["scrypt"] 
  
  sha256String := sha256.New()
  sha256String.Write([]byte(password.(string)))
  
  sha256Final := sha256String.Sum(nil)
  hmac := hmac.New(sha256.New,salt)
  hmac.Write([]byte(password.(string)))
  hmacFinal := hmac.Sum(nil)

 y := pbkdf2.Key([]byte(password.(string)),salt,rounds,64 , sha256.New)
 
  // fmt.Println(string(rounds))
  fmt.Printf(hex.EncodeToString(sha256Final) + "\n")
  fmt.Printf(hex.EncodeToString(hmacFinal) + "\n")
  fmt.Printf(hex.EncodeToString(y) + "\n")
} 


