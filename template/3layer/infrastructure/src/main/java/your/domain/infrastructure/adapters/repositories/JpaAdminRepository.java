package {{yourDomain}}.infrastructure.adapters.repositories;

import {{yourDomain}}.application.ports.repositories.AdminRepository;
import {{yourDomain}}.core.domain.entities.Admin;
import {{yourDomain}}.infrastructure.adapters.repositories.entities.AdminEntity;
import {{yourDomain}}.infrastructure.adapters.repositories.jpa.AdminJpaRepository;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public class JpaAdminRepository implements AdminRepository {
    private final AdminJpaRepository jpaRepository;

    public JpaAdminRepository(AdminJpaRepository jpaRepository) {
        this.jpaRepository = jpaRepository;
    }

    @Override
    public Optional<Admin> findByEmail(String email) {
        return jpaRepository.findByEmail(email)
                .map(this::toDomain);
    }

    @Override
    public Admin save(Admin admin) {
        AdminEntity entity = toEntity(admin);
        AdminEntity savedEntity = jpaRepository.save(entity);
        return toDomain(savedEntity);
    }

    @Override
    public Optional<Admin> findById(Long id) {
        return jpaRepository.findById(id)
                .map(this::toDomain);
    }

    private Admin toDomain(AdminEntity entity) {
        return new Admin(
                entity.getId(),
                entity.getEmail(),
                entity.getPassword(),
                entity.getName(),
                entity.getCreatedAt(),
                entity.getUpdatedAt()
        );
    }

    private AdminEntity toEntity(Admin domain) {
        AdminEntity entity = new AdminEntity();
        entity.setId(domain.getId());
        entity.setEmail(domain.getEmail());
        entity.setPassword(domain.getPassword());
        entity.setName(domain.getName());
        entity.setCreatedAt(domain.getCreatedAt());
        entity.setUpdatedAt(domain.getUpdatedAt());
        return entity;
    }
}
