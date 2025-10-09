package {{yourDomain}}.application.usecases;

import {{yourDomain}}.application.ports.repositories.{{UserEntity}}Repository;
import {{yourDomain}}.application.ports.services.PasswordService;
import {{yourDomain}}.application.ports.services.TokenService;
import {{yourDomain}}.core.domain.entities.{{UserEntity}};
import {{yourDomain}}.core.domain.valueobjects.AuthenticationResult;

import java.util.Optional;

public class Authenticate{{UserEntity}}UseCase {

    private final {{UserEntity}}Repository {{userEntity}}Repository;
    private final PasswordService passwordService;
    private final TokenService tokenService;

    public Authenticate{{UserEntity}}UseCase({{UserEntity}}Repository {{userEntity}}Repository,
                                     PasswordService passwordService,
                                     TokenService tokenService) {
        this.{{userEntity}}Repository = {{userEntity}}Repository;
        this.passwordService = passwordService;
        this.tokenService = tokenService;
    }

    public Optional<AuthenticationResult> execute(String email, String password) {
        Optional<{{UserEntity}}> {{userEntity}}Opt = {{userEntity}}Repository.findByEmail(email);

        if ({{userEntity}}Opt.isEmpty()) {
            return Optional.empty();
        }

        {{UserEntity}} {{userEntity}} = {{userEntity}}Opt.get();

        if (!passwordService.matches(password, {{userEntity}}.getPassword())) {
            return Optional.empty();
        }

        String token = tokenService.generateToken({{userEntity}}.getEmail(), {{userEntity}}.getRole());

        return Optional.of(new AuthenticationResult(
                token,
                {{userEntity}}.getRole(),
                {{userEntity}}.getEmail(),
                {{userEntity}}.getName()
        ));
    }

}
