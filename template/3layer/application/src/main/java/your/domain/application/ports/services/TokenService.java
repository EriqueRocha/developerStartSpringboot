package {{yourDomain}}.application.ports.services;

public interface TokenService {
    String generateToken(String email, String role);
    boolean validateToken(String token);
    String extractEmail(String token);
    String extractRole(String token);
}
