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
  - Citar a relevância de transformadas espaciais, e sua história (DCT, Wavelet, Walsh-Hadamard^[webp]).
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

- _RD Curve_ (_Rate Distortion Curve_)^[webp2]: curva de distorção de píxeis.
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

## Justificativa

<!--
Comentários:

-->

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

## Conclusão?

<!--
Comentários:

-->

[webp]: ["VP8 Data Format and Decoding Guide" (PDF). 23 September 2010. Retrieved 2 October 2010. [permanent dead link]](https://en.wikipedia.org/wiki/WebP)
[webp2]: [https://chromium.googlesource.com/codecs/libwebp2/](https://chromium.googlesource.com/codecs/libwebp2/)
