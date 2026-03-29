// Tasks: 
// La IA debe poder generar backlogs a partir de lo hablado en el chat y lo planeado en artifacts
// Los backlogs deben ser editables y aprobados por el usuario antes de comenzar a ejecutarse
// De tener tareas de bajo acoplamiento (con poca relacion) y ser factible la division de dominios
// debe poder usar `branches` o `worktrees`` dependiendo de:
// - La dificultad
// - El acoplamiento de los files relacionados
// - Los planes hechos en chat y artifacts
// Estas divisiones son para multitasking, de forma que el backlog se pueda dividir en sub backlogs
// Uno por cada agent session, ya este trabajando en un branch o un worktree
// Si el backlog no es muy largo o dificil, o por el contrario, las tareas tienen demasiada relacion
// Se debe sacrificar el multitasking para dedicar todos los recursos a un avance lineal en las tasks