UFABC 2025 Q2 - Projeto Dirigido

# Análise (comparativa e qualitativa) de métodos de codificação de imagens

| ----- | ----- |
| :---- | :---- |
| **Nome** | Guilherme AC Zaluchi |
| **Orientador** | Carlos S Santos |

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

**Definição de imagem digital e sua representação.**
<!-- Conferir livro na biblioteca que tem uma definição sucinta de imagem digital, e usá-la aqui. -->

**Definição de codificação de imagens, e sua importância.**

**O quê se sabe:**
- Citar relações espaciais entre píxeis de imagens não-caóticas.
  - Definir "imagens caóticas" e "imagens aleatórias".
  - Citar a relevância de transformadas espaciais, e sua história (DCT, Wavelet, Walsh-Hadamard^[1][webp]).
- Técnicas singulares que sejam muito efetivas para casos específicos, não são boas generalistas (trazer fontes para esse argumento).
- Combinações de técnicas singulares específicas são boas generalistas (citar a especificação PNG? Talvez o vídeo sobre PNG e as palavras do autor que escolheu "a soma do módulo da aplicação de cada técnica" como meio de escolher a técnica mais adequada).

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

- _RD Curve_ (_Rate Distortion Curve_)^[2][webp2]: curva de taxa de distorção de píxeis.
- PSNR (_Peak Signal-to-Noise Ratio_)
- SSIM (_S. S. I. M._)
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

## Metodologia

<!--
Comentários:

-->

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

[webp]: https://en.wikipedia.org/wiki/WebP
[webp2]: https://chromium.googlesource.com/codecs/libwebp2/
