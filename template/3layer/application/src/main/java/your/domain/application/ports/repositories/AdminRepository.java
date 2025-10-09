package {{yourDomain}}.application.ports.repositories;

import {{yourDomain}}.core.domain.entities.Admin;

import java.util.Optional;

public interface AdminRepository {
    Optional<Admin> findByEmail(String email);
    Admin save(Admin admin);
    Optional<Admin> findById(Long id);
}
