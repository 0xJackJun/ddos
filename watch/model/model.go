package model

type Response struct {
	Result  bool   `json:"result"`
	Code    string `json:"code"`
	Message string `json:"message"`
}

type Ddos struct {
	Email string `json:"email"`
}
