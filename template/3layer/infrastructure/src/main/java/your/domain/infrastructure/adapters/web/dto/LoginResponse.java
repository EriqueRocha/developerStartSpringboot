package {{yourDomain}}.infrastructure.adapters.web.dto;

public class LoginResponse {
    private String message;
    private String role;
    private String email;
    private String name;

    public LoginResponse() {}

    public LoginResponse(String message, String role, String email, String name) {
        this.message = message;
        this.role = role;
        this.email = email;
        this.name = name;
    }

    public String getMessage() {
        return message;
    }

    public void setMessage(String message) {
        this.message = message;
    }

    public String getRole() {
        return role;
    }

    public void setRole(String role) {
        this.role = role;
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }
}
