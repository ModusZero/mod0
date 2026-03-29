// TODO Definir, aqui se guardara una cache del contexto construido antes de una request a la IA
// De forma que lo que casi nunca cambia se mantenga aqui ya estructurado (rules, skills generales, etc)
// y el resto se inyecta en cada request (contexto dinamico, como el chat history reciente, artifacts recientes, etc)