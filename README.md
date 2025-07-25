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
O problema de codificação de imagens se origina na necessidade de representar, de forma eficiente, informações visuais oriundas de diversas fontes - como fotografia convencional, composições gráficas e análises espectrais. No contexto atual, em que se tornam viáveis imagens com resoluções cada vez maiores, e também o surgimento de novas imagens se mantém em crescente (taxas anuais de 6% à 8%, no aumento do número de fotografias realizadas[^photutorial_photo_statistics]), a busca por melhores técnicas de compressão, que equilibrem fidelidade à imagem original e eficiência de armazenamento, é um campo dinâmico e ativo.

Atualmente, o campo da compressão de imagens de origem genérica é majoritariamente dominado por formatos como JPEG, PNG e WebP, cada um empregando conjuntos de estratégias distintas: transformadas espaciais (DCT no JPEG, Walsh-Hadamard no WebP), divisão em blocos, e combinações de abordagens híbridas (PNG, WebP), todas se aproveitando de relações espaciais, locais ou globais, da composição da imagem, a fim de reduzir a dimensionalidade necessária para representá-la, assim reduzindo o uso de banda (de disco ou de rede).
E como um campo em desenvolvimento ativo, em que algoritmos novos continuam surgindo, a pesquisa em codificação de imagens mantém atividade impulsionada por diversos fatores, como: (1) demandas emergentes em aplicações (*streaming*), (2) o surgimento de novas arquiteturas computacionais, e (3) avanços em técnicas baseadas em aprendizado de máquina[^clic2025].


Esta proposta diferencia-se de estudos comparativos por visar estabelecer uma metodologia para análise e comparação dentre técnicas de compressão de imagens, respondendo à lacuna já identificada[^aim_2024] sobre a inadequação dos métodos atuais para comparar técnicas heterogêneas de forma genérica.

O trabalho está organizado em cinco seções principais. Após esta introdução, a segunda seção apresenta a revisão da literatura, abordando desde conceitos fundamentais de representação de imagens até técnicas comuns de compressão. A terceira seção detalha a justificativa científica e prática da pesquisa, enquanto a quarta seção explicita os objetivos gerais e específicos. A metodologia proposta é descrita na quinta seção, seguida pelo cronograma de execução na sexta seção. Por fim, as considerações finais sintetizam as contribuições esperadas deste estudo.

No restante do documento, serão apresentadas a revisão da literatura considerada, e a justificativa da presente proposta.

## Revisão da literatura

<!--
Comentários:

-->

### Conceitos e Definições Fundamentais

<!--
Comentários:

-->

>In digital image processing systems, one usually deals with arrays of numbers obtained by spatially sampling points of a physical image. After processing, another array of numbers is produced, and these numbers are then used to reconstruct a continuous image for viewing.[^digitalimageprocessing]

Imagens digitais, quando não comprimidas, são comumente compostas como uma simples sequência (ou matriz) de números, representando uma amostragem espacial de pontos de uma imagem física, através dos píxeis, intensidades luminosas em diferentes canais de cores para cada ponto da imagem digital.[^digitalimageprocessing]

#### A relevância do campo de codificação de imagens

A codificação (e a compressão) de imagens refere-se ao processo de representar uma imagem digital de forma compacta, ocupando pouco espaço digital de armazenamento ou banda de transmissão, mas mantendo-se a capacidade de reconstruir uma versão *suficientemente próxima* da imagem original. Essa técnica é fundamental em sistemas modernos de comunicação e armazenamento, onde a eficiência na utilização de recursos é crucial.

#### Conhecimento atual

A fim de se simplificar os escopos de busca para métodos eficazes de compressão de imagens, sempre se busca reconhecer relações que diminuam a dimensionalidade de escopo de busca de soluções.

Dentre estas relações, está o fato de que, para imagens não-caóticas (imagens caóticas possuem baixa correlação espacial entre informações, como imagens aleatórias) costumam existir relações espaciais discerníveis entre píxeis de uma região vizinha duma imagem.  
Uma forma de se explorar esta relação algoritmicamente, costumam ser as transformadas espaciais: dado uma sub-imagem, que é um bloco de píxeis recortado da imagem original, um procedimento do gênero costuma ser genericamente análogas a (por simplicidade, outras fases de compactação foram deixadas de lado no exemplo):
1. Aplicar uma transformada espacial que evidencie informações relevantes, como valor mediano e degradês.
2. Realizar uma redução de dimensionalidade, ou *quantização*.
3. Salvar o resultado, ocupando menos espaço em memória.

Para se reconstruir a imagem, basta:
1. Desfazer a redução de dimensionalidade, recompondo a estrutura do bloco (como um preenchimento por zeros).
2. Aplicar a transformada inversa.

Uma das transformadas mais comumente utilizadas (formato JPEG[^branch_education_jpeg]), DCT (*Discrete Cosine Transformation*), ou Transformada de Cosseno Discreta, é um bom exemplo:
- Aplicando a transformada de cosseno, a fase de redução de dimensionalidade se trata de dividir cada respectivo valor da matriz resultado para parâmetros específicos pré-determinados, arredondar os resultados, e por fim, estruturá-los como uma sequência linear. Com isso, o resultado costuma constar de grandes sequências do valor $0$, facilmente compressível por outros métodos genéricos.
- A recomposição é feita ao se reestruturar os dados como matriz, os multiplicando pelos respectivos fatores, e então chegando ao resultado reconstruído.

Algoritmos mais complexos fazem uso de mais de uma técnica de compressão, escolhendo a que parecer (dada alguma heurística pré-determinada) mais eficiente: PNG testa várias técnicas, e escolhe aquela cuja soma do módulo dos valores comprimidos for a menor[^reducible_png], por exemplo.

#### Métodos de comparação de imagens

Existem diversos métodos de comparação de imagens, ou *comparação de sinais*. PSNR (_Peak Signal-to-Noise Ratio_), SSIM (_Structural Similarity Index Measure_, ou Índice de Medida de Similaridade Estrutural), técnicas de *hash* (que convertem um sinal em um valor numérico, transformando o cálculo de distâncias entre sinais em distâncias entre valores).

Todos os citados foram criados, tendo como um dos objetivos, comparar diferenças entre imagens. Essas técnicas são tidas como aproximações que visam aproximar a capacidade de comparação da percepção humana, apenas confirmada através de testes de comparação subjetiva, que demandam tempo e recursos humanos para realizar as comparações.[^wikipedia_subjetctive_quality]

### Justificativa

<!-- Um texto científico deve, internamente, justificar sua realização. -->

O campo de pesquisa de codificação de imagens assume papel estratégico na otimização de recursos físicos e computacionais, onde avanços em algoritmos de compressão impactam diretamente a eficiência no uso de banda de armazenamento e transferência de dados, tendo como efeitos: (1) redução de custos operacionais em infraestruturas, (2) diminuição de latência computacional, e (3) mitigação de perdas de informação - tanto em transmissões quanto em algoritmos de compressão destrutiva (_lossy compression_ / *compressão com perdas*).

No contexto tecnológico, com a evolução em dispositivos de captura (sensores com resoluções crescentes) e reprodução (telas de alta definição), combinada com a democratização do acesso a tecnologias de imagem digital (segundo dados da , o volume global de imagens digitais geradas cresce a taxa anual de 6% a 8%[^photutorial_photo_statistics]) há uma constante demanda por algoritmos mais eficientes.

Este estudo comparativo justifica-se pela necessidade emergente de metodologias de avaliação adaptáveis ao cenário dinâmico de compressão de imagens, onde:

1. A demanda por novas técnicas se mantém ativa. A exemplo, a competição anual por novos métodos de compressão de imagens CLIC[^clic2025], financiada por algumas das maiores empresas do ramo.
2. As métricas tradicionais de avaliação (PSNR, SSIM) serem limitadas em sua capacidade de representar a percepção humana[^aim_2024].
3. A escolha ótima de técnica varia conforme aplicação (telemedicina, *streaming*, armazenamento em nuvem, arquivologia etc.) 

Ao propor uma metodologia sistemática de avaliação, este trabalho busca preencher a lacuna entre o desenvolvimento acelerado de novos algoritmos e a carência de ferramentas robustas para sua comparação objetiva, particularmente em cenários de uso realístico.

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
A metodologia proposta envolve três etapas principais:
1. seleção de conjuntos de imagens representativos (*datasets*) para realizar as comparações;
2. escolha criteriosa de algoritmos de comparação de imagens que abranjam diferentes abordagens;
3. e execução sistemática de processos de compressão seguidos de coleta e análise de métricas de desempenho.
Esta abordagem permitirá uma avaliação abrangente e objetiva das técnicas estudadas.

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
[^DigitalImageProcessing]: William K. Pratt. *Digital Image Processing : PIKS Scientific Inside / William K. Pratt*, 4th edition.
[^aim_2024]: *AIM 2024 Challenge on Compressed Video Quality Assessment: Methods and Results* https://arxiv.org/abs/2408.11982.
[^photutorial_photo_statistics]: Matic Broz. *Photo statistics: How many photos are taken every day?*, atualizado em 21/05/2025. https://photutorial.com/photos-statistics/.
[^clic2025]: *7th Challenge on Learned Image Compression*. https://clic2025.compression.cc/
[^branch_education_jpeg]: *How are Images Compressed? \[46MB ↘↘ 4.07MB\] JPEG In Depth*. https://www.youtube.com/watch?v=Kv1Hiv3ox8I
[^reducible_png]: *How PNG Works: Compromising Speed for Quality*. https://www.youtube.com/watch?v=EFUYNoFRHQI
[^wikipedia_subjetctive_quality]: *Subjective video quality*. Wikipédia. https://en.wikipedia.org/wiki/Subjective_video_quality