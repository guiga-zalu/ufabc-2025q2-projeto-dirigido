UFABC 2025 Q2 - Projeto Dirigido

# Análise (comparativa e qualitativa) de métodos de codificação de imagens

| .              | **Nome**             |
| :------------- | :------------------- |
| **Aluno**      | Guilherme AC Zaluchi |
| **Orientador** | Carlos S Santos      |

## Resumo

<!--
Comentários:

-->

## Abstract

<!--
Comentários:

-->

## Introdução

<!--
Comentários:

-->

## Revisão da literatura

<!--
Comentários:

-->

### Definição do problema

<!--
Comentários:

-->

**Imagem digital e sua representação**
>In digital image processing systems, one usually deals with arrays of numbers obtained by spatially sampling points of a physical image. After processing, another array of numbers is produced, and these numbers are then used to reconstruct a continuous image for viewing.[^digitalimageprocessing]

Imagens digitais, quando não comprimidas, são comumente compostas como uma simples sequência (ou matriz) de números, representando uma amostragem espacial de pontos de uma imagem física, através dos píxeis, intensidades luminosas em diferentes canais de cores para cada ponto da imagem digital. **anexar fontes**

**Codificação de imagens, e a relevância do campo**

A codificação (e a compressão) de imagens refere-se ao processo de representar uma imagem digital de forma compacta, ocupando pouco espaço digital de armazenamento ou banda de transmissão, mas mantendo-se a capacidade de reconstruir uma versão *suficientemente próxima* da imagem original. Essa técnica é fundamental em sistemas modernos de comunicação e armazenamento, onde a eficiência na utilização de recursos é crucial. **anexar fontes**

**Conhecimento atual**
A fim de se simplificar os escopos de busca para métodos eficazes de compressão de imagens, sempre se busca reconhecer relações que diminuam a dimensionalidade de escopo de busca de soluções.

Dentre estas relações, está o fato de que, para imagens não-caóticas **anexar fontes, definir imagens não-caóticas, ou imagens caóticas** costumam existir relações espaciais discerníveis entre píxeis de uma região vizinha uma imagem.  
Uma forma de se explorar esta relação algoritmicamente, costumam ser as transformadas espaciais: dado uma sub-imagem, um bloco de píxeis recortado da imagem original, um procedimento do gênero costuma ser genericamente análogas a:
1. aplicar uma transformada espacial que evidencie informações relevantes **explicar isso**
2. realizar uma redução de dimensionalidade
3. salvar o resultado, ocupando menos espaço em memória

Para se reconstruir a imagem, basta:
1. desfazer a redução de dimensionalidade, recompondo a estrutura do bloco (preenchendo com zeros) **reescrever isso, está ruim**
2. reverter a transformação

DCT (*Discrete Cosine Transformation*), ou Transformada de Cosseno Discreta, é ...

<!--
As relações **anexar fontes**
- Citar relações espaciais entre píxeis de imagens não-caóticas. Até 2 parágrafos.
  - Definir "imagens caóticas" e "imagens aleatórias".
  - Citar a relevância de transformadas espaciais, e sua história (DCT, Wavelet, Walsh-Hadamard[^webp]).
- Técnicas singulares que sejam muito efetivas para casos específicos, não são boas generalistas (trazer fontes para esse argumento).
- Combinações de técnicas singulares específicas são boas generalistas (citar a especificação PNG? Talvez o vídeo sobre PNG e as palavras do autor que escolheu "a soma do módulo da aplicação de cada técnica" como meio de escolher a técnica mais adequada).
-->

**O quê se tem em aberto:**  
Me esqueci do quê colocar aqui.

**Exemplos de métodos atuais:**  
Citar as técnicas de:
- quebra de blocos em PNG, JPEG e WEBP
- transformada em JPEG e WEBP
- combinações de técnicas em PNG e WEBP

**Métodos de comparação de algoritmos:**  
Expandir textos, garantindo citações para todos.

Citar [https://en.wikipedia.org/wiki/Subjective_video_quality](https://en.wikipedia.org/wiki/Subjective_video_quality), e como os métodos de comparação tentam aproximar a comparação humana.

- _RD Curve_ (_Rate Distortion Curve_)[^webp2]: curva de taxa de distorção de píxeis.
- PSNR (_Peak Signal-to-Noise Ratio_)
- SSIM (_S. S. I. M._)
- MS SSIM
- VMAF (_V. M. A. F._)
- Técnicas de _hash_:
  - O quê efetivamente é _hash_
    - _Hash_ de imagens. Me basear em [https://www.reddit.com/r/AskTechnology/comments/sqzn2h/explain_like_im_5_differences_between_a_b_d_phash/](https://www.reddit.com/r/AskTechnology/comments/sqzn2h/explain_like_im_5_differences_between_a_b_d_phash/), e buscar as fontes.
  - a-hash
  - b-hash
  - d-hash
  - p-hash
  - $\alpha$-hash

### Justificativa

<!--
Comentários:

-->
<!-- Um texto científico deve, internamente, justificar sua realização. -->

- Porquê este tópico é relevante?
	- Algoritmos melhores implicam em menos banda de disco e rede $\rightarrow$ menos custos, menos perda de dados (para algoritmos destrutivos)
	- ¿Melhor aproveitamento de hardware, como gasto mais eficiente de CPU (talvez isso seja só coisa de implementação)?
- Como este estudo em específico é relevante?
	- Quais são os problemas atuais?
	- Como este estudo difere de outros? Ler vários outros e citá-los aqui.

## Objetivo geral e objetivos específicos

<!--
Comentários:

-->
A presente proposta tem, como objetivos principais:
- Avaliar métodos e técnicas de codificação de imagens.
- Determinar uma metodologia de comparação entre diferentes técnicas de codificação de imagens.
Com estes objetivos, ...

Para atingir estes objetivos, a proposta tem como objetivos específicos:
- Identificar algoritmos e técnicas de compressão de imagens comumente utilizados para armazenamento e para transmissão.
- Identificar grupos de técnicas comuns, dentre algoritmos comumente utilizados, e algoritmos utilizados em competições e desafios de compressão de imagens e vídeo.
- Realizar comparações entre os algoritmos e técnicas.
- Analisar comparativamente os resultados de compressão (imagens) e os resultados métricos (lista de números).


¿Conferir algoritmos e técnicas pouco explorados, ou as usadas nas competições, e ...?

## Metodologia

<!--
Comentários:
Criar um rascunho. O quê eu preciso fazer para resolver isso (isso := desafio da proposta)?
-->
A metodologia proposta envolve três etapas principais: seleção de conjuntos de imagens representativos (datasets) para realizar as comparações; escolha criteriosa de algoritmos de comparação de imagens que abranjam diferentes abordagens; e execução sistemática de processos de compressão seguidos de coleta e análise de métricas de desempenho. Esta abordagem permitirá uma avaliação abrangente e objetiva das técnicas estudadas.

- Selecionar um ou mais conjuntos (*datasets*) de imagens para realizar comparações.
- Selecionar algoritmos de comparação de imagens.
- Realizar compressão das imagens e coletar métricas sobre os resultados da comparação.

## Cronograma de execução

<!--
Comentários:

-->

Até 14/07, coletar literatura de referência.  
Até 21/07, ?

## Conclusão?

<!--
Comentários:

-->

# Bibliografia

[^webp]: https://en.wikipedia.org/wiki/WebP
[^webp2]: https://chromium.googlesource.com/codecs/libwebp2/
[^DigitalImageProcessing]: # William K. Pratt. Digital Image Processing : PIKS Scientific Inside / William K. Pratt, 4th edition.
