package {{yourDomain}}.infrastructure.adapters.repositories.jpa;

import {{yourDomain}}.infrastructure.adapters.repositories.entities.AdminEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface AdminJpaRepository extends JpaRepository<AdminEntity, Long> {
    Optional<AdminEntity> findByEmail(String email);
}
