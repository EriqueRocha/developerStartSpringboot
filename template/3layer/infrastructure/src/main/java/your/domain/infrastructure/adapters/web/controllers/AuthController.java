package {{yourDomain}}.infrastructure.adapters.web.controllers;

import {{yourDomain}}.application.usecases.AuthenticateAdminUseCase;
import {{yourDomain}}.application.usecases.Authenticate{{UserEntity}}UseCase;
import {{yourDomain}}.core.domain.valueobjects.AuthenticationResult;
import {{yourDomain}}.infrastructure.adapters.web.dto.LoginRequest;
import {{yourDomain}}.infrastructure.adapters.web.dto.LoginResponse;
import jakarta.servlet.http.Cookie;
import jakarta.servlet.http.HttpServletResponse;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Optional;

@RestController
@RequestMapping("/auth")
public class AuthController {
    private final Authenticate{{UserEntity}}UseCase authenticate{{UserEntity}}UseCase;
    private final AuthenticateAdminUseCase authenticateAdminUseCase;

    public AuthController(Authenticate{{UserEntity}}UseCase authenticate{{UserEntity}}UseCase,
                          AuthenticateAdminUseCase authenticateAdminUseCase) {
        this.authenticate{{UserEntity}}UseCase = authenticate{{UserEntity}}UseCase;
        this.authenticateAdminUseCase = authenticateAdminUseCase;
    }

    @PostMapping("/{{userEntity}}/login")
    public ResponseEntity<LoginResponse> login{{UserEntity}}(@RequestBody LoginRequest request,
                                                     HttpServletResponse response) {
        Optional<AuthenticationResult> result = authenticate{{UserEntity}}UseCase.execute(
                request.getEmail(),
                request.getPassword()
        );

        if (result.isEmpty()) {
            return ResponseEntity.badRequest()
                    .body(new LoginResponse("Invalid credentials", null, null, null));
        }

        AuthenticationResult authResult = result.get();
        setTokenCookie(response, authResult.getToken());

        return ResponseEntity.ok(new LoginResponse(
                "Login successful",
                authResult.getRole(),
                authResult.getEmail(),
                authResult.getName()
        ));
    }

    @PostMapping("/admin/login")
    public ResponseEntity<LoginResponse> loginAdmin(@RequestBody LoginRequest request,
                                                    HttpServletResponse response) {
        Optional<AuthenticationResult> result = authenticateAdminUseCase.execute(
                request.getEmail(),
                request.getPassword()
        );

        if (result.isEmpty()) {
            return ResponseEntity.badRequest()
                    .body(new LoginResponse("Invalid credentials", null, null, null));
        }

        AuthenticationResult authResult = result.get();
        setTokenCookie(response, authResult.getToken());

        return ResponseEntity.ok(new LoginResponse(
                "Login successful",
                authResult.getRole(),
                authResult.getEmail(),
                authResult.getName()
        ));
    }

    @PostMapping("/logout")
    public ResponseEntity<String> logout(HttpServletResponse response) {
        Cookie cookie = new Cookie("token", null);
        cookie.setMaxAge(0);
        cookie.setHttpOnly(true);
        cookie.setPath("/");
        response.addCookie(cookie);

        return ResponseEntity.ok("Logout successful");
    }

    private void setTokenCookie(HttpServletResponse response, String token) {
        Cookie cookie = new Cookie("token", token);
        cookie.setHttpOnly(true);
        cookie.setSecure(false); // Set to true in production with HTTPS
        cookie.setPath("/");
        cookie.setMaxAge(24 * 60 * 60); // 24 hours
        response.addCookie(cookie);
    }
}
