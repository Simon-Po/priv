Set-Location -Path "C:\Jenkins Monitoring"
Start-Process curl.exe -ArgumentList '-o agent.jar -sO http://wiptransfer01:8080/jnlpJars/agent.jar' -Wait
java -jar agent.jar -url http://wiptransfer01:8080/ -secret 329db03fc68b83d4e2c29d6b94caece039ae913683bc6a73ee4e4d072f867459 -name "Monitoring Node" -workDir "C:\Jenkins Monitoring"