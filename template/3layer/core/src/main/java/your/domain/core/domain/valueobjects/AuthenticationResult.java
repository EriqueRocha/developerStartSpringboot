package {{yourDomain}}.core.domain.valueobjects;

public class AuthenticationResult {
    private final String token;
    private final String role;
    private final String email;
    private final String name;

    public AuthenticationResult(String token, String role, String email, String name) {
        this.token = token;
        this.role = role;
        this.email = email;
        this.name = name;
    }

    public String getToken() {
        return token;
    }

    public String getRole() {
        return role;
    }

    public String getEmail() {
        return email;
    }

    public String getName() {
        return name;
    }
}
