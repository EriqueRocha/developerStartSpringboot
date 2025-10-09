package {{yourDomain}}.application.usecases;

import {{yourDomain}}.application.ports.repositories.AdminRepository;
import {{yourDomain}}.application.ports.services.PasswordService;
import {{yourDomain}}.application.ports.services.TokenService;
import {{yourDomain}}.core.domain.entities.Admin;
import {{yourDomain}}.core.domain.valueobjects.AuthenticationResult;

import java.util.Optional;

public class AuthenticateAdminUseCase {

    private final AdminRepository adminRepository;
    private final PasswordService passwordService;
    private final TokenService tokenService;

    public AuthenticateAdminUseCase(AdminRepository adminRepository,
                                    PasswordService passwordService,
                                    TokenService tokenService) {
        this.adminRepository = adminRepository;
        this.passwordService = passwordService;
        this.tokenService = tokenService;
    }

    public Optional<AuthenticationResult> execute(String email, String password) {
        Optional<Admin> adminOpt = adminRepository.findByEmail(email);

        if (adminOpt.isEmpty()) {
            return Optional.empty();
        }

        Admin admin = adminOpt.get();

        if (!passwordService.matches(password, admin.getPassword())) {
            return Optional.empty();
        }

        String token = tokenService.generateToken(admin.getEmail(), admin.getRole());

        return Optional.of(new AuthenticationResult(
                token,
                admin.getRole(),
                admin.getEmail(),
                admin.getName()
        ));
    }

}
