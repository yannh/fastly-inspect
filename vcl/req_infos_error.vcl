if (obj.status == 601) {
  set obj.status = 200;
  set obj.response = "OK";
  set obj.http.Access-Control-Allow-Origin = "*";
  set obj.http.Content-Type = "application/json";
  synthetic {"{
    "cwnd": "} client.socket.cwnd {",
    "nexthop": ""} client.socket.nexthop {"",
    "rtt": "} client.socket.tcpi_rtt {",
    "delta_retrans": "} client.socket.tcpi_delta_retrans {",
    "total_retrans": "} client.socket.tcpi_total_retrans {",
    "fastly_server_ip": ""} server.ip {"",
    "client_ip": ""} client.ip {"",
    "client_as_name": ""} client.as.name {"",
    "client_as_number": ""} client.as.number {"",
    "city": ""} client.geo.city {"",
    "continent": ""} client.geo.continent_code {"",
    "country": ""} client.geo.country_name {"",
    "region": ""} client.geo.region {""
  }"};
  return(deliver);
}