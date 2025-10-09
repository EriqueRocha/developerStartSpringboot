package {{yourDomain}}.infrastructure.adapters.web.controllers;

import {{yourDomain}}.application.usecases.Create{{UserEntity}}UseCase;
import {{yourDomain}}.core.domain.entities.{{UserEntity}};
import {{yourDomain}}.infrastructure.adapters.web.dto.Create{{UserEntity}}Request;
import {{yourDomain}}.infrastructure.adapters.web.dto.Create{{UserEntity}}Response;
import jakarta.validation.Valid;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/{{userEntity}}s")
public class {{UserEntity}}Controller {
    private final Create{{UserEntity}}UseCase create{{UserEntity}}UseCase;

    public {{UserEntity}}Controller(Create{{UserEntity}}UseCase create{{UserEntity}}UseCase) {
        this.create{{UserEntity}}UseCase = create{{UserEntity}}UseCase;
    }

    @PostMapping
    public ResponseEntity<Create{{UserEntity}}Response> create{{UserEntity}}(@Valid @RequestBody Create{{UserEntity}}Request request) {
        Optional<{{UserEntity}}> result = create{{UserEntity}}UseCase.execute(
                request.getEmail(),
                request.getPassword(),
                request.getName()
        );

        if (result.isEmpty()) {
            return ResponseEntity.status(HttpStatus.CONFLICT)
                    .body(new Create{{UserEntity}}Response(
                            "Email already exists",
                            null,
                            null,
                            null
                    ));
        }

        {{UserEntity}} created{{UserEntity}} = result.get();
        return ResponseEntity.status(HttpStatus.CREATED)
                .body(new Create{{UserEntity}}Response(
                        "{{UserEntity}} created successfully",
                        created{{UserEntity}}.getId(),
                        created{{UserEntity}}.getEmail(),
                        created{{UserEntity}}.getName()
                ));
    }
}
