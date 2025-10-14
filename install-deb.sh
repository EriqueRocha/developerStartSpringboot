#!/bin/bash
set -e

#Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== Developer Start Spring Boot - Instalador ===${NC}\n"

#Verificar se está rodando como root
if [ "$EUID" -ne 0 ]; then 
  echo -e "${RED}Por favor, execute com sudo${NC}"
  exit 1
fi

#Detectar arquitetura
ARCH=$(dpkg --print-architecture 2>/dev/null || echo "amd64")
echo -e "${YELLOW}Arquitetura detectada: ${ARCH}${NC}"

if [ "$ARCH" != "amd64" ]; then
  echo -e "${RED}Apenas amd64 é suportado no momento.${NC}"
  exit 1
fi

#Verificar se é Debian/Ubuntu
if ! command -v apt-get &> /dev/null; then
  echo -e "${RED}Este instalador funciona apenas em sistemas Debian/Ubuntu.${NC}"
  exit 1
fi

echo -e "${YELLOW}Instalando dependências...${NC}"
apt-get update -qq
apt-get install -y -qq curl gnupg ca-certificates > /dev/null 2>&1

#Instalar chave GPG
echo -e "${YELLOW}Instalando chave GPG...${NC}"
KEYRING_PATH="/usr/share/keyrings/developerstartspringboot-archive-keyring.gpg"
curl -fsSL https://eriquerocha.github.io/developerStartSpringboot/public.key | gpg --dearmor -o "$KEYRING_PATH"

if [ ! -f "$KEYRING_PATH" ]; then
  echo -e "${RED}Falha ao instalar a chave GPG.${NC}"
  exit 1
fi

echo -e "${GREEN}✓ Chave GPG instalada${NC}"

#Adicionar repositório
echo -e "${YELLOW}Adicionando repositório APT...${NC}"
SOURCES_FILE="/etc/apt/sources.list.d/developerstartspringboot.list"
echo "deb [signed-by=${KEYRING_PATH}] https://eriquerocha.github.io/developerStartSpringboot stable main" > "$SOURCES_FILE"

if [ ! -f "$SOURCES_FILE" ]; then
  echo -e "${RED}Falha ao adicionar repositório.${NC}"
  exit 1
fi

echo -e "${GREEN}✓ Repositório adicionado${NC}"

#Atualizar cache do APT
echo -e "${YELLOW}Atualizando cache do APT...${NC}"
apt-get update -qq

#Instalar o pacote
echo -e "${YELLOW}Instalando developerstartspringboot...${NC}"
apt-get install -y developerstartspringboot

#Verificar instalação
if command -v dss &> /dev/null; then
  echo -e "\n${GREEN}✓ Instalação concluída com sucesso!${NC}"
  echo -e "${GREEN}Execute 'dss init' para começar.${NC}\n"
  dss --version 2>/dev/null || echo ""
else
  echo -e "${RED}Instalação concluída, mas o comando 'dss' não foi encontrado.${NC}"
  exit 1
fi
