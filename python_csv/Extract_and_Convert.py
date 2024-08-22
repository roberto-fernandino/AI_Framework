import os
import pandas as pd
import gzip
import shutil


def extract_gz_files(gz_folder, extract_to):
    """
    Descomprime todos os arquivos .gz na pasta especificada para o diretório de destino e
    converte arquivos .tsv extraídos para .csv.
    
    :param gz_folder: Caminho para a pasta contendo arquivos .gz.
    :param extract_to: Caminho para o diretório onde os arquivos serão extraídos e convertidos.
    """
    # Verifique se o diretório de origem existe
    if not os.path.exists(gz_folder):
        print(f"O diretório de origem {gz_folder} não existe.")
        return

    # Crie o diretório de destino se não existir
    if not os.path.exists(extract_to):
        os.makedirs(extract_to)
        print(f"Diretório de destino {extract_to} criado.")

    # Verifique o conteúdo do diretório de origem
    gz_files = [file_name for file_name in os.listdir(gz_folder) if file_name.endswith('.gz')]
    if not gz_files:
        print(f"Nenhum arquivo .gz encontrado em {gz_folder}.")
        return

    print(f"Arquivos .gz encontrados: {gz_files}")

    # Iterar sobre todos os arquivos .gz na pasta
    for file_name in gz_files:
        print(f"Processando {file_name}...")
        gz_path = os.path.join(gz_folder, file_name)
        tsv_path = os.path.join(extract_to, file_name.replace('.gz', ''))

        # Verificar se o arquivo já existe
        if os.path.exists(tsv_path):
            print(f"Arquivo {tsv_path} já existe, pulando descompactação.")
            continue
        # Descompactar o arquivo .gz
        try:
            with gzip.open(gz_path, 'rb') as f_in:
                with open(tsv_path, 'wb') as f_out:
                    shutil.copyfileobj(f_in, f_out)
            print(f"Descompactado {gz_path} para {tsv_path}")

            # Converta .tsv para .csv
            csv_path = tsv_path.replace('.tsv', '.csv')
            df = pd.read_csv(tsv_path, sep='\t')
            df.to_csv(csv_path, index=False)
            print(f"Convertido {tsv_path} para {csv_path}")
            # Removendo arquivo .tsv e deixando apenas o .csv
            os.remove(tsv_path)

        
        except Exception as e:
            print(f"Erro ao processar {gz_path}: {e}")

# Exemplo de uso
gz_folder = 'btc'  # Pasta com arquivos .gz
extract_to = 'btc_transactions'  # Diretório onde os arquivos serão extraídos e convertidos

# Chame a função para descompactar os arquivos .gz e converter para .csv
extract_gz_files(gz_folder, extract_to)





