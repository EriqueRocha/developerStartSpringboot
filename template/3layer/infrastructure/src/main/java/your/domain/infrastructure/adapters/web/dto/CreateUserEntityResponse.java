package {{yourDomain}}.infrastructure.adapters.web.dto;

public class Create{{UserEntity}}Response {
    private String message;
    private Long id;
    private String email;
    private String name;

    public Create{{UserEntity}}Response() {}

    public Create{{UserEntity}}Response(String message, Long id, String email, String name) {
        this.message = message;
        this.id = id;
        this.email = email;
        this.name = name;
    }

    public String getMessage() {
        return message;
    }

    public void setMessage(String message) {
        this.message = message;
    }

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
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
